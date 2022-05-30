import { type } from "os";
import { useForm } from "react-hook-form";
import { object, string, number, InferType } from "yup";

export const CustomForm = () => {
  let infoSchema = object({
    name: string()
      .required()
      .matches(/^[A-Za-z]+$/, "Must be only chars"),
    age: number().required().positive().integer(),
    address: string().required(),
  });

  type Info = InferType<typeof infoSchema>;

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm();
  const onSubmit = (data: Info) => console.log(data);

  return (
    <form
      onSubmit={handleSubmit(async (data) => {
        try {
          let validatedData = await infoSchema.validate(data);
          onSubmit(validatedData);
        } catch (err) {
          alert(err.errors);
          //   err.name; // => 'ValidationError'
          //   err.errors; // => ['Deve ser maior que 18']
        }
      })}
    >
      <label>
        Name:
        <input {...register("name", { required: true })} />
        {errors.name && <p>Please enter your name.</p>}
        <br />
      </label>

      <label>
        Age:
        <input {...register("age", { required: true })} />
        {/* <input {...register("age", { pattern: /\d+/, required: true })} /> */}
        {errors.age && <p>Please enter number for age.</p>}
        <br />
      </label>

      <label>
        Address:
        <input {...register("address", { required: true })} />
        {errors.address && <p>address is required.</p>}
        <br />
      </label>

      <input type="submit" />
    </form>
  );
};
